use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 4V2" ></ path > < path d = "M15 16v-2" ></ path > < path d = "M8 9h2" ></ path > < path d = "M20 9h2" ></ path > < path d = "M17.8 11.8 19 13" ></ path > < path d = "M15 9h.01" ></ path > < path d = "M17.8 6.2 19 5" ></ path > < path d = "m3 21 9-9" ></ path > < path d = "M12.2 6.2 11 5" ></ path > < / > } } pub const LUCIDE_WAND : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;