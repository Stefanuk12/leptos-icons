use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m14 5-3 3 2 7 8-8-7-2Z" ></ path > < path d = "m14 5-3 3-3-3 3-3 3 3Z" ></ path > < path d = "M9.5 6.5 4 12l3 6" ></ path > < path d = "M3 22v-2c0-1.1.9-2 2-2h4a2 2 0 0 1 2 2v2H3Z" ></ path > < / > } } pub const LUCIDE_LAMP_DESK : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24")] } ;