use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cx = "9" cy = "7" r = "4" ></ circle > < line y1 = "11" x2 = "16" x1 = "22" y2 = "11" ></ line > < / > } } pub const LUCIDE_USER_MINUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("fill" , "none")] } ;