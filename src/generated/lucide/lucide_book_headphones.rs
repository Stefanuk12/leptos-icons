use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" ></ path > < circle cx = "9" r = "1" cy = "12" ></ circle > < path d = "M8 12v-2a4 4 0 0 1 8 0v2" ></ path > < circle cy = "12" r = "1" cx = "15" ></ circle > < / > } } pub const LUCIDE_BOOK_HEADPHONES : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24")] } ;