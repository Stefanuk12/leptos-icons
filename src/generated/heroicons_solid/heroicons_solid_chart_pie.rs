use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2.25 13.5a8.25 8.25 0 0 1 8.25-8.25.75.75 0 0 1 .75.75v6.75H18a.75.75 0 0 1 .75.75 8.25 8.25 0 0 1-16.5 0Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < path clip - rule = "evenodd" d = "M12.75 3a.75.75 0 0 1 .75-.75 8.25 8.25 0 0 1 8.25 8.25.75.75 0 0 1-.75.75h-7.5a.75.75 0 0 1-.75-.75V3Z" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_CHART_PIE : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;