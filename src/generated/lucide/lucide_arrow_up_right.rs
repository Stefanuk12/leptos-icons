use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 7h10v10" ></ path > < path d = "M7 17 17 7" ></ path > < / > } } pub const LUCIDE_ARROW_UP_RIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;