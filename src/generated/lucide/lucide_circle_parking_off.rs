use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "m5 5 14 14" ></ path > < path d = "M13 13a3 3 0 1 0 0-6H9v2" ></ path > < path d = "M9 17v-2.34" ></ path > < / > } } pub const LUCIDE_CIRCLE_PARKING_OFF : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;