use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 4h6l3 7H8l3-7Z" ></ path > < path d = "M14 11v5a2 2 0 0 1-2 2H8" ></ path > < path d = "M4 15h2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H4v-6Z" ></ path > < / > } } pub const LUCIDE_LAMP_WALL_UP : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;