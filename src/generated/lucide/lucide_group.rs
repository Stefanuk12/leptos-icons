use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 7V5c0-1.1.9-2 2-2h2" ></ path > < path d = "M17 3h2c1.1 0 2 .9 2 2v2" ></ path > < path d = "M21 17v2c0 1.1-.9 2-2 2h-2" ></ path > < path d = "M7 21H5c-1.1 0-2-.9-2-2v-2" ></ path > < rect y = "7" width = "7" rx = "1" x = "7" height = "5" ></ rect > < rect width = "7" height = "5" x = "10" y = "12" rx = "1" ></ rect > < / > } } pub const LUCIDE_GROUP : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;