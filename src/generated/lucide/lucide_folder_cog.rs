use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "18" r = "3" ></ circle > < path d = "M10.3 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v3.3" ></ path > < path d = "m21.7 19.4-.9-.3" ></ path > < path d = "m15.2 16.9-.9-.3" ></ path > < path d = "m16.6 21.7.3-.9" ></ path > < path d = "m19.1 15.2.3-.9" ></ path > < path d = "m19.6 21.7-.4-1" ></ path > < path d = "m16.8 15.3-.4-1" ></ path > < path d = "m14.3 19.6 1-.4" ></ path > < path d = "m20.7 16.8 1-.4" ></ path > < / > } } pub const LUCIDE_FOLDER_COG : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;