use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 7V5c0-1.1.9-2 2-2h2" ></ path > < path d = "M17 3h2c1.1 0 2 .9 2 2v2" ></ path > < path d = "M21 17v2c0 1.1-.9 2-2 2h-2" ></ path > < path d = "M7 21H5c-1.1 0-2-.9-2-2v-2" ></ path > < rect x = "7" width = "7" rx = "1" y = "7" height = "5" ></ rect > < rect x = "10" y = "12" height = "5" rx = "1" width = "7" ></ rect > < / > } } pub const LUCIDE_GROUP : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;