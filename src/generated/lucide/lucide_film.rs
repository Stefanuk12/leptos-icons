use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" width = "18" height = "18" x = "3" ></ rect > < path d = "M7 3v18" ></ path > < path d = "M3 7.5h4" ></ path > < path d = "M3 12h18" ></ path > < path d = "M3 16.5h4" ></ path > < path d = "M17 3v18" ></ path > < path d = "M17 7.5h4" ></ path > < path d = "M17 16.5h4" ></ path > < / > } } pub const LUCIDE_FILM : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;