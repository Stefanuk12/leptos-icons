use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 3 4 7l4 4" ></ path > < path d = "M4 7h16" ></ path > < path d = "m16 21 4-4-4-4" ></ path > < path d = "M20 17H4" ></ path > < / > } } pub const LUCIDE_ARROW_LEFT_RIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;