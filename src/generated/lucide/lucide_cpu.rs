use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "16" height = "16" rx = "2" y = "4" x = "4" ></ rect > < rect y = "9" height = "6" rx = "1" width = "6" x = "9" ></ rect > < path d = "M15 2v2" ></ path > < path d = "M15 20v2" ></ path > < path d = "M2 15h2" ></ path > < path d = "M2 9h2" ></ path > < path d = "M20 15h2" ></ path > < path d = "M20 9h2" ></ path > < path d = "M9 2v2" ></ path > < path d = "M9 20v2" ></ path > < / > } } pub const LUCIDE_CPU : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;