use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "12" cx = "12" ></ circle > < path d = "M4.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-.5" ></ path > < path d = "M4.5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-.5" ></ path > < path d = "M6 6h.01" ></ path > < path d = "M6 18h.01" ></ path > < path d = "m15.7 13.4-.9-.3" ></ path > < path d = "m9.2 10.9-.9-.3" ></ path > < path d = "m10.6 15.7.3-.9" ></ path > < path d = "m13.6 15.7-.4-1" ></ path > < path d = "m10.8 9.3-.4-1" ></ path > < path d = "m8.3 13.6 1-.4" ></ path > < path d = "m14.7 10.8 1-.4" ></ path > < path d = "m13.4 8.3-.3.9" ></ path > < / > } } pub const LUCIDE_SERVER_COG : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;