use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 15h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 17" ></ path > < path d = "m7 21 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9" ></ path > < path d = "m2 16 6 6" ></ path > < circle cx = "16" r = "2.9" cy = "9" ></ circle > < circle cx = "6" cy = "5" r = "3" ></ circle > < / > } } pub const LUCIDE_HAND_COINS : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;