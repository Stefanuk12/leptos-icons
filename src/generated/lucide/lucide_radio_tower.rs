use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4.9 16.1C1 12.2 1 5.8 4.9 1.9" ></ path > < path d = "M7.8 4.7a6.14 6.14 0 0 0-.8 7.5" ></ path > < circle cx = "12" r = "2" cy = "9" ></ circle > < path d = "M16.2 4.8c2 2 2.26 5.11.8 7.47" ></ path > < path d = "M19.1 1.9a9.96 9.96 0 0 1 0 14.1" ></ path > < path d = "M9.5 18h5" ></ path > < path d = "m8 22 4-11 4 11" ></ path > < / > } } pub const LUCIDE_RADIO_TOWER : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;