use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "3" cy = "17" ></ circle > < path d = "M4.2 15.1A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2" ></ path > < path d = "m15.7 18.4-.9-.3" ></ path > < path d = "m9.2 15.9-.9-.3" ></ path > < path d = "m10.6 20.7.3-.9" ></ path > < path d = "m13.1 14.2.3-.9" ></ path > < path d = "m13.6 20.7-.4-1" ></ path > < path d = "m10.8 14.3-.4-1" ></ path > < path d = "m8.3 18.6 1-.4" ></ path > < path d = "m14.7 15.8 1-.4" ></ path > < / > } } pub const LUCIDE_CLOUD_COG : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;