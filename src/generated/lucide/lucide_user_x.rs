use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cy = "7" cx = "9" r = "4" ></ circle > < line y2 = "13" x1 = "17" y1 = "8" x2 = "22" ></ line > < line y2 = "13" x1 = "22" y1 = "8" x2 = "17" ></ line > < / > } } pub const LUCIDE_USER_X : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;