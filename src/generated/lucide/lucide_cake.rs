use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8" ></ path > < path d = "M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1" ></ path > < path d = "M2 21h20" ></ path > < path d = "M7 8v3" ></ path > < path d = "M12 8v3" ></ path > < path d = "M17 8v3" ></ path > < path d = "M7 4h0.01" ></ path > < path d = "M12 4h0.01" ></ path > < path d = "M17 4h0.01" ></ path > < / > } } pub const LUCIDE_CAKE : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;