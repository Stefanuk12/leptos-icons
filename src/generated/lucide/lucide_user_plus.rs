use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cy = "7" r = "4" cx = "9" ></ circle > < line y2 = "14" x1 = "19" y1 = "8" x2 = "19" ></ line > < line x1 = "22" x2 = "16" y1 = "11" y2 = "11" ></ line > < / > } } pub const LUCIDE_USER_PLUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2")] } ;