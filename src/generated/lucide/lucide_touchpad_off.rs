use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16" ></ path > < path d = "M2 14h12" ></ path > < path d = "M22 14h-2" ></ path > < path d = "M12 20v-6" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M22 16V6a2 2 0 0 0-2-2H10" ></ path > < / > } } pub const LUCIDE_TOUCHPAD_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24")] } ;