use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a8 8 0 0 0-16 0" ></ path > < circle cx = "10" cy = "8" r = "5" ></ circle > < path d = "M22 20c0-3.37-2-6.5-4-8a5 5 0 0 0-.45-8.3" ></ path > < / > } } pub const LUCIDE_USERS_ROUND : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;