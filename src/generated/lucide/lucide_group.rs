use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 7V5c0-1.1.9-2 2-2h2" ></ path > < path d = "M17 3h2c1.1 0 2 .9 2 2v2" ></ path > < path d = "M21 17v2c0 1.1-.9 2-2 2h-2" ></ path > < path d = "M7 21H5c-1.1 0-2-.9-2-2v-2" ></ path > < rect width = "7" y = "7" height = "5" rx = "1" x = "7" ></ rect > < rect height = "5" width = "7" rx = "1" y = "12" x = "10" ></ rect > < / > } } pub const LUCIDE_GROUP : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;