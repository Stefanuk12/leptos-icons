use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < rect width = "6" y = "9" height = "6" x = "9" ></ rect > < / > } } pub const LUCIDE_CIRCLE_STOP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;