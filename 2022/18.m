%% Reading input
lava_list = readmatrix("input.txt");
lava_list = lava_list + 2; % border of size 1
lava_cell = num2cell(lava_list);
maxdims = max(lava_list)+1; % border of size 1
lava_tensor = zeros(maxdims);
for i=1:length(lava_cell)
    lava_tensor(lava_cell{i,:}) = 1;
end

%% Part 1: Convolution to count free sides
KERNEL1 =        [0 0 0;
                  0 -1 0;
                  0 0 0];
KERNEL1(:,:,2) = [0 -1 0;
                  -1 6 -1;
                   0 -1 0],
KERNEL1(:,:,3) = [0 0 0;
                  0 -1 0;
                  0 0 0];

lava_conv = convn(lava_tensor, KERNEL1, 'same');
sum(lava_conv(lava_conv>0))

%% Part 2: Let the gas spread using BFS
gas = [];
gas_candidates = [maxdims];
while ~isempty(gas_candidates)

    gas_candidate=gas_candidates(end,:);
    if isempty(gas) || ~ismember(gas_candidate, gas, "rows")
        gas = [gas; gas_candidate];
    end
    gas_candidates = gas_candidates(1:end-1,:);
    length(gas_candidates)

    nbs = neighbors(gas_candidate);
    for i=1:length(nbs)
        nb = nbs(i,:);
        if 1<=nb(1) && nb(1) <= maxdims(1) && ...
           1<=nb(2) && nb(2) <= maxdims(2) && ...
           1<=nb(3) && nb(3) <= maxdims(3) && ...
           ~ismember(nb, lava_list, 'rows') && ...
           ~ismember(nb, gas, 'rows')
                gas_candidates = [gas_candidates; nb];
        end
    end
end

%% Part 2: Counting free sides of gas inside hull
gas_cell = num2cell(gas);
gas_tensor = zeros(maxdims);
for i=1:length(gas_cell)
    gas_tensor(gas_cell{i,:}) = 1;
end

hull_conv = convn(~gas_tensor, KERNEL1, 'same');
sum(hull_conv(hull_conv>0))

%% Visualization
visu = lava_tensor + 2*(~gas_tensor)

%% Find all neighboring points
function result = neighbors(coord)
    x = coord(1);
    y = coord(2);
    z = coord(3);
    result = [x-1, y, z;
              x+1, y, z;
              x, y-1, z;
              x, y+1, z;
              x, y, z-1;
              x, y, z+1];
end