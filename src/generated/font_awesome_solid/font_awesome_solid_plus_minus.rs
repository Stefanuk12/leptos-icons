use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M224 32c0-17.7-14.3-32-32-32s-32 14.3-32 32V144H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H160V320c0 17.7 14.3 32 32 32s32-14.3 32-32V208H336c17.7 0 32-14.3 32-32s-14.3-32-32-32H224V32zM0 480c0 17.7 14.3 32 32 32H352c17.7 0 32-14.3 32-32s-14.3-32-32-32H32c-17.7 0-32 14.3-32 32z" ></ path > < / > } } pub const FontAwesomeSolidPlusMinus : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 384 512")] } ;