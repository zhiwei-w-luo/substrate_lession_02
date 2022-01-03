use crate::mock::{Event as TestEvent, new_test_ext, KittiesModule, Origin, System, Test};
use frame_support::{assert_ok, assert_noop};
use super::*;


/// For Test works case
#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_eq!(KittiesCount::<Test>::get(), Some(1));
		assert_eq!(Owner::<Test>::get(0), Some(1));
		assert_has_event!(Event::<Test>::KittyCreated(1,0));
	});
}
#[test]
fn breed_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
		assert_eq!(KittiesCount::<Test>::get(), Some(3));
		assert_has_event!(Event::<Test>::KittyCreated(1, 2));
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {

		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_ok!(KittiesModule::transfer(Origin::signed(account_id), 2, 0));

		assert_eq!(Owner::<Test>::get(0), Some(2));
		assert_has_event!(Event::<Test>::KittyTransferred(1, 2, 0));
	});
}

#[test]
fn sell_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let price: u128 = 1_200;
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));
		assert_eq!(ListForSale::<Test>::get(0), Some(price));
		assert_has_event!(Event::<Test>::KittyListed(1, 0, Some(price)));
	});
}


/// For Test Failed case

#[test]
fn create_failed_if_kittiescount_overflow() {
	new_test_ext().execute_with(|| {
		KittiesCount::<Test>::put(u32::max_value());
		let account_id: u64 = 1;
		assert_noop!(KittiesModule::create(Origin::signed(account_id)), Error::<Test>::KittiesCountOverflow);
	});
}

#[test]
fn create_failed_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 3;
		assert_noop!(KittiesModule::create(Origin::signed(account_id)), Error::<Test>::NotEnoughBalanceForSave);

	});
}



#[test]
fn breed_failed_if_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		assert_noop!(KittiesModule::breed(Origin::signed(3), 0, 1), Error::<Test>::NotEnoughBalanceForSave);
	});
}

#[test]
fn breed_failed_if_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 3), Error::<Test>::InvalidKittyIndex);
	});
}

#[test]
fn breed_failed_if_same_parent() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 0), Error::<Test>::SameParentIndex);
	});
}



#[test]
fn transfer_failed_if_not_owner() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_noop!(KittiesModule::transfer(Origin::signed(2u64), 3, 0), Error::<Test>::NotOwner);
	});
}

#[test]
fn transfer_failed_if_new_owner_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id)));
		assert_noop!(KittiesModule::transfer(Origin::signed(account_id), 3, 0), Error::<Test>::NotEnoughBalanceForSave);
	});
}


#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(KittiesModule::create(Origin::signed(1)));

		let price: u128 = 1_200;
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));
		assert_ok!(KittiesModule::buy(Origin::signed(2), 0));
		assert_eq!(Owner::<Test>::get(0), Some(2));

		assert_has_event!(Event::<Test>::KittyTransferred(1, 2, 0));
	});
}

#[test]
fn buy_failed_if_buyer_is_same_owner() {
	new_test_ext().execute_with(|| {

		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let price: u128 = 1_200;
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));

		assert_noop!(KittiesModule::buy(Origin::signed(1), 0), Error::<Test>::BuyerIsOwner);
	});
}

#[test]
fn buy_failed_if_not_for_sale() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, None));
		assert_noop!(KittiesModule::buy(Origin::signed(2), 0), Error::<Test>::NotForSale);
	});
}

#[test]
fn buy_failed_if_not_enough_balance() {
	new_test_ext().execute_with(|| {

		let price: u128 = 1_200;
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::sell(Origin::signed(1), 0, Some(price)));
		assert_noop!(KittiesModule::buy(Origin::signed(3), 0), Error::<Test>::NotEnoughBalanceForBuying);
	});
}


#[test]
fn sell_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let price: u128 = 1_200;
		assert_noop!(KittiesModule::sell(Origin::signed(3), 0, Some(price)), Error::<Test>::NotOwner);
	});
}
