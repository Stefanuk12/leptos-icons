use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a8 8 0 0 0-16 0" ></ path > < circle r = "5" cx = "10" cy = "8" ></ circle > < path d = "M22 20c0-3.37-2-6.5-4-8a5 5 0 0 0-.45-8.3" ></ path > < / > } } pub const LUCIDE_USERS_ROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;