use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 7h-9" ></ path > < path d = "M14 17H5" ></ path > < circle cx = "17" r = "3" cy = "17" ></ circle > < circle cy = "7" cx = "7" r = "3" ></ circle > < / > } } pub const LUCIDE_SETTINGS_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24")] } ;