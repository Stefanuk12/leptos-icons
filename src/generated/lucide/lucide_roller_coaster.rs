use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 19V5" ></ path > < path d = "M10 19V6.8" ></ path > < path d = "M14 19v-7.8" ></ path > < path d = "M18 5v4" ></ path > < path d = "M18 19v-6" ></ path > < path d = "M22 19V9" ></ path > < path d = "M2 19V9a4 4 0 0 1 4-4c2 0 4 1.33 6 4s4 4 6 4a4 4 0 1 0-3-6.65" ></ path > < / > } } pub const LUCIDE_ROLLER_COASTER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;