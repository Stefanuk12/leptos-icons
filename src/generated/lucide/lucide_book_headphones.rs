use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" ></ path > < path d = "M8 12v-2a4 4 0 0 1 8 0v2" ></ path > < circle cy = "12" r = "1" cx = "15" ></ circle > < circle cy = "12" r = "1" cx = "9" ></ circle > < / > } } pub const LUCIDE_BOOK_HEADPHONES : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;