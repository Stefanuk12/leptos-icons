use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19 8.5V10h1.5A1.5 1.5 0 1 0 19 8.5" ></ path > < rect rx = "1.5" x = "8" height = "8" width = "3" y = "14" ></ rect > < path d = "M5 15.5V14H3.5A1.5 1.5 0 1 0 5 15.5" ></ path > < rect y = "13" height = "3" x = "14" rx = "1.5" width = "8" ></ rect > < path d = "M15.5 19H14v1.5a1.5 1.5 0 1 0 1.5-1.5" ></ path > < rect rx = "1.5" width = "8" x = "2" height = "3" y = "8" ></ rect > < path d = "M8.5 5H10V3.5A1.5 1.5 0 1 0 8.5 5" ></ path > < / > } } pub const LucideSlack : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;