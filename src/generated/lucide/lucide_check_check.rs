use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 6 7 17l-5-5" ></ path > < path d = "m22 10-7.5 7.5L13 16" ></ path > < / > } } pub const LUCIDE_CHECK_CHECK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;