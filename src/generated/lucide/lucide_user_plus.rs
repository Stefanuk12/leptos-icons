use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" ></ path > < circle cx = "9" cy = "7" r = "4" ></ circle > < line x2 = "19" y2 = "14" y1 = "8" x1 = "19" ></ line > < line x1 = "22" y2 = "11" y1 = "11" x2 = "16" ></ line > < / > } } pub const LUCIDE_USER_PLUS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;