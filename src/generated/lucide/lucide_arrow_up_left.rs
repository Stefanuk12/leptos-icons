use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 17V7h10" ></ path > < path d = "M17 17 7 7" ></ path > < / > } } pub const LUCIDE_ARROW_UP_LEFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;