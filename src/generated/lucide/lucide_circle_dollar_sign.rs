use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "10" ></ circle > < path d = "M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" ></ path > < path d = "M12 18V6" ></ path > < / > } } pub const LUCIDE_CIRCLE_DOLLAR_SIGN : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;