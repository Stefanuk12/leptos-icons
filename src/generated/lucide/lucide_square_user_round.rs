use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a6 6 0 0 0-12 0" ></ path > < circle cy = "11" r = "4" cx = "12" ></ circle > < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ></ rect > < / > } } pub const LUCIDE_SQUARE_USER_ROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;