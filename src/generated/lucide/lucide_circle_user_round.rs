use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 20a6 6 0 0 0-12 0" ></ path > < circle cx = "12" cy = "10" r = "4" ></ circle > < circle r = "10" cx = "12" cy = "12" ></ circle > < / > } } pub const LucideCircleUserRound : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;