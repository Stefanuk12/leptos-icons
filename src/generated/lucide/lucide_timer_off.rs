use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 2h4" ></ path > < path d = "M4.6 11a8 8 0 0 0 1.7 8.7 8 8 0 0 0 8.7 1.7" ></ path > < path d = "M7.4 7.4a8 8 0 0 1 10.3 1 8 8 0 0 1 .9 10.2" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M12 12v-2" ></ path > < / > } } pub const LUCIDE_TIMER_OFF : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;