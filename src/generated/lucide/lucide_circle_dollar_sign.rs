use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "10" cy = "12" ></ circle > < path d = "M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" ></ path > < path d = "M12 18V6" ></ path > < / > } } pub const LUCIDE_CIRCLE_DOLLAR_SIGN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none")] } ;