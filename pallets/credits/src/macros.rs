macro_rules! create_typed_class_getter_and_setter_macros {
  (
    $d:tt,

    $macro_name:ident,
    $get_name:ident,
    $set_name:ident
    $(, $instance_name:ident: $instance_ty:ty )?
  ) => {
    macro_rules! $macro_name {
      (@mandatory $get:ident, $set:ident, $ty:ty) => {
        #[cfg(test)]
        fn $get(
          class: &CreditId
          $(, $instance_name: $instance_ty )?
        ) -> Result<$ty, DispatchError> {
          T::Card::$get_name(class $(, $instance_name )?, &stringify!($get))
            .ok_or_else(|| Error::<T>::CardAttributeDoesNotExist.into())
        }

        fn $set(
          class: &CreditId
          $(, $instance_name: $instance_ty )?,
          value: &$ty
        ) -> DispatchResult {
          T::Card::$set_name(class $(, $instance_name )?, &stringify!($get), value)
        }
      };
      (@optional $get:ident, $set:ident, $ty:ty) => {
        fn $get(class: &CreditId $(, $instance_name: $instance_ty )?) -> Option<$ty> {
          T::Card::$get_name(class $(, $instance_name )?, &stringify!($get))
        }

        fn $set(
          class: &CreditId
          $(, $instance_name: $instance_ty )?,
          value: &Option<$ty>
        ) -> DispatchResult {
          if let Some(ref elem) = *value {
            T::Card::$set_name(class $(, $instance_name )?, &stringify!($get), elem)
          }
          else {
            T::Card::$set_name::<_, Option<$ty>>(class $(, $instance_name )?, &stringify!($get), &None)
          }
        }
      };
    }
  };
}

create_typed_class_getter_and_setter_macros!(
  $,
  create_typed_class_getter_and_setter,
  typed_class_attribute,
  set_typed_class_attribute
);
create_typed_class_getter_and_setter_macros!(
  $,
  create_typed_getter_and_setter,
  typed_attribute,
  set_typed_attribute,
  instance: &InstanceId
);
