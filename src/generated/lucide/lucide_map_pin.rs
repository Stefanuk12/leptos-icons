use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z" ></ path > < circle cy = "10" cx = "12" r = "3" ></ circle > < / > } } pub const LucideMapPin : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;