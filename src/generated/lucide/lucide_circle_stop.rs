use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < rect width = "6" height = "6" x = "9" y = "9" ></ rect > < / > } } pub const LUCIDE_CIRCLE_STOP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;