use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 13h6l3 7H8l3-7Z" ></ path > < path d = "M14 13V8a2 2 0 0 0-2-2H8" ></ path > < path d = "M4 9h2a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H4v6Z" ></ path > < / > } } pub const LUCIDE_LAMP_WALL_DOWN : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;