use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" height = "18" x = "3" rx = "2" ></ rect > < path d = "M7 3v18" ></ path > < path d = "M3 7.5h4" ></ path > < path d = "M3 12h18" ></ path > < path d = "M3 16.5h4" ></ path > < path d = "M17 3v18" ></ path > < path d = "M17 7.5h4" ></ path > < path d = "M17 16.5h4" ></ path > < / > } } pub const LUCIDE_FILM : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;