use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "4" width = "16" height = "16" rx = "2" y = "4" ></ rect > < rect y = "9" width = "6" x = "9" height = "6" ></ rect > < path d = "M15 2v2" ></ path > < path d = "M15 20v2" ></ path > < path d = "M2 15h2" ></ path > < path d = "M2 9h2" ></ path > < path d = "M20 15h2" ></ path > < path d = "M20 9h2" ></ path > < path d = "M9 2v2" ></ path > < path d = "M9 20v2" ></ path > < / > } } pub const LUCIDE_CPU : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;