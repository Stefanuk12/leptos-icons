use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "1" ></ circle > < path d = "M20.2 20.2c2.04-2.03.02-7.36-4.5-11.9-4.54-4.52-9.87-6.54-11.9-4.5-2.04 2.03-.02 7.36 4.5 11.9 4.54 4.52 9.87 6.54 11.9 4.5Z" ></ path > < path d = "M15.7 15.7c4.52-4.54 6.54-9.87 4.5-11.9-2.03-2.04-7.36-.02-11.9 4.5-4.52 4.54-6.54 9.87-4.5 11.9 2.03 2.04 7.36.02 11.9-4.5Z" ></ path > < / > } } pub const LUCIDE_ATOM : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;