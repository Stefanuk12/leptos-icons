use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "15" cx = "18" ></ circle > < circle r = "4" cx = "9" cy = "7" ></ circle > < path d = "M10 15H6a4 4 0 0 0-4 4v2" ></ path > < path d = "m21.7 16.4-.9-.3" ></ path > < path d = "m15.2 13.9-.9-.3" ></ path > < path d = "m16.6 18.7.3-.9" ></ path > < path d = "m19.1 12.2.3-.9" ></ path > < path d = "m19.6 18.7-.4-1" ></ path > < path d = "m16.8 12.3-.4-1" ></ path > < path d = "m14.3 16.6 1-.4" ></ path > < path d = "m20.7 13.8 1-.4" ></ path > < / > } } pub const LUCIDE_USER_COG : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;