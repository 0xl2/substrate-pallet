use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_not_throw_errors() {
	new_test_ext().execute_with(|| {
		let title = "Novel title".as_bytes().to_vec();
		let title1 = "Other Novel".as_bytes().to_vec();

		// create
		assert_ok!(TemplateModule::create_novel(Origin::signed(1), title));
		assert_noop!(TemplateModule::create_novel(Origin::signed(1), "Novel title".as_bytes().to_vec()), Error::<Test>::NovelAlreadyCreated);

		// read
		assert_noop!(TemplateModule::read_novel(Origin::signed(1), "Other Novel".as_bytes().to_vec()), Error::<Test>::NoSuchNovel);
		assert_noop!(TemplateModule::read_novel(Origin::signed(2), "Novel title".as_bytes().to_vec()), Error::<Test>::NotNovelOwner);
		assert_ok!(TemplateModule::read_novel(Origin::signed(1), "Novel title".as_bytes().to_vec()));

		// update
		assert_noop!(TemplateModule::update_novel(Origin::signed(1), "Other Novel".as_bytes().to_vec()), Error::<Test>::NoSuchNovel);
		assert_noop!(TemplateModule::update_novel(Origin::signed(2), "Novel title".as_bytes().to_vec()), Error::<Test>::NotNovelOwner);
		assert_ok!(TemplateModule::update_novel(Origin::signed(1), title1));

		// remove
		assert_noop!(TemplateModule::remove_novel(Origin::signed(1), "Novel title".as_bytes().to_vec()), Error::<Test>::NoSuchNovel);
		assert_noop!(TemplateModule::remove_novel(Origin::signed(2), "Other Novel".as_bytes().to_vec()), Error::<Test>::NotNovelOwner);
		assert_ok!(TemplateModule::remove_novel(Origin::signed(1), "Other Novel".as_bytes().to_vec()));
	});
}

#[test]
fn it_works_for_update_novel() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::update_novel(Origin::signed(1), vec![1]));
	});
}

#[test]
fn it_works_for_read_novel() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::read_novel(Origin::signed(1), vec![1]));
	});
}

#[test]
fn it_works_for_remove_novel() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::remove_novel(Origin::signed(1), vec![1]));
	});
}