use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "13" rx = "1.5" width = "3" height = "8" y = "2" ></ rect > < path d = "M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5" ></ path > < rect width = "3" y = "14" x = "8" height = "8" rx = "1.5" ></ rect > < path d = "M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5" ></ path > < rect width = "8" y = "13" height = "3" rx = "1.5" x = "14" ></ rect > < path d = "M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5" ></ path > < rect height = "3" y = "8" x = "2" width = "8" rx = "1.5" ></ rect > < path d = "M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5" ></ path > < / > } } pub const LUCIDE_SLACK : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;