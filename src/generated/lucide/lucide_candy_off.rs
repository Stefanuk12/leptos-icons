use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m8.5 8.5-1 1a4.95 4.95 0 0 0 7 7l1-1" ></ path > < path d = "M11.843 6.187A4.947 4.947 0 0 1 16.5 7.5a4.947 4.947 0 0 1 1.313 4.657" ></ path > < path d = "M14 16.5V14" ></ path > < path d = "M14 6.5v1.843" ></ path > < path d = "M10 10v7.5" ></ path > < path d = "m16 7 1-5 1.367.683A3 3 0 0 0 19.708 3H21v1.292a3 3 0 0 0 .317 1.341L22 7l-5 1" ></ path > < path d = "m8 17-1 5-1.367-.683A3 3 0 0 0 4.292 21H3v-1.292a3 3 0 0 0-.317-1.341L2 17l5-1" ></ path > < line y1 = "2" x1 = "2" y2 = "22" x2 = "22" ></ line > < / > } } pub const LUCIDE_CANDY_OFF : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;