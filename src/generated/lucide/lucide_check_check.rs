use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 6 7 17l-5-5" ></ path > < path d = "m22 10-7.5 7.5L13 16" ></ path > < / > } } pub const LUCIDE_CHECK_CHECK : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;