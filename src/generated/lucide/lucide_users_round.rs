use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 21a8 8 0 0 0-16 0" ></ path > < circle cy = "8" cx = "10" r = "5" ></ circle > < path d = "M22 20c0-3.37-2-6.5-4-8a5 5 0 0 0-.45-8.3" ></ path > < / > } } pub const LUCIDE_USERS_ROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;