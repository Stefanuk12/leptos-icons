use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a6 6 0 0 0-12 0" ></ path > < circle cx = "12" cy = "11" r = "4" ></ circle > < rect width = "18" x = "3" y = "3" rx = "2" height = "18" ></ rect > < / > } } pub const LUCIDE_SQUARE_USER_ROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;