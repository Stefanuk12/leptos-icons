use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1.5" x = "13" height = "8" y = "2" width = "3" ></ rect > < path d = "M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5" ></ path > < rect y = "14" x = "8" width = "3" rx = "1.5" height = "8" ></ rect > < path d = "M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5" ></ path > < rect rx = "1.5" width = "8" y = "13" x = "14" height = "3" ></ rect > < path d = "M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5" ></ path > < rect y = "8" width = "8" height = "3" rx = "1.5" x = "2" ></ rect > < path d = "M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5" ></ path > < / > } } pub const LUCIDE_SLACK : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;