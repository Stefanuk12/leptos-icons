use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 15h14" ></ path > < path d = "M5 9h14" ></ path > < path d = "m14 20-5-5 6-6-5-5" ></ path > < / > } } pub const LUCIDE_RAIL_SYMBOL : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;